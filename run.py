import click
import asyncio
import subprocess
import json
import os
import shutil

PIPE = asyncio.subprocess.PIPE

QUARRY = os.path.dirname(os.path.abspath(__file__))
SERVICES = ["oracle"]


async def start_pm2_service(name, command, *args, cwd=None):
    proc = await asyncio.create_subprocess_exec(
        "pm2",
        "start",
        command,
        "--name",
        name,
        "--",
        *args,
        cwd=cwd,
    )
    return proc


async def wait_process_running(
    process: str, success_log_str: str, log_lines=15
) -> None:
    while True:
        proc = await asyncio.create_subprocess_exec(
            "pm2",
            "logs",
            process,
            "--nostream",
            f"--lines={log_lines}",
            stdout=PIPE,
            stderr=PIPE,
        )
        out, _ = await proc.communicate()
        if success_log_str in out.decode("utf-8"):
            break
        else:
            await asyncio.sleep(0.1)


async def pm2_delete_process(process: str):
    proc = await asyncio.create_subprocess_exec(
        "pm2", "delete", process, stdout=PIPE, stderr=PIPE
    )
    await proc.wait()


async def check_process_running(process_name: str) -> bool:
    proc = await asyncio.create_subprocess_exec(
        "pm2", "jlist", stdout=PIPE, stderr=PIPE
    )
    out, _ = await proc.communicate()
    try:
        processes = json.loads(out.decode("utf-8"))
    except json.decoder.JSONDecodeError:
        print(f"failed to parse processes {out.decode('utf-8')}")
        raise
    for process in processes:
        if process["name"] != process_name:
            continue
        status = process["pm2_env"]["status"]
        if status != "online":
            print(f"found {process} process, but it's not running, deleting...")
            await pm2_delete_process(process)
            break
        return True
    return False


async def display_pm2_logs(service_name):
    proc = await asyncio.create_subprocess_exec(
        "pm2", "logs", service_name, stdout=PIPE, stderr=PIPE
    )

    # Async read stdout and stderr
    async def read_stream(stream, callback):
        while True:
            line = await stream.readline()
            if line:
                callback(line.decode().strip())
            else:
                break

    # Define a simple print callback
    def print_line(line):
        print(line)

    # Await both stdout and stderr
    await asyncio.gather(
        read_stream(proc.stdout, print_line), read_stream(proc.stderr, print_line)
    )

    await proc.wait()


async def run_local_evm() -> None:
    """
    Start the anvil local node for locally deploying smart contracts. The evm
    is deployed on port 8545.
    """
    is_running = await check_process_running("local-evm")
    if is_running:
        print("local-evm running")
        return
    proc = await asyncio.create_subprocess_exec(
        "pm2",
        "start",
        "anvil",
        "--name",
        "local-evm",
        "--",
        "--chain-id",
        "1337",
        "--port",
        "8545",
        "--code-size-limit",
        "9999999999999",
    )
    await proc.wait()

    started_log = "Listening on"

    print(f"starting local-evm...")
    await wait_process_running("local-evm", started_log)
    print(f"local-evm running")


async def deploy_contracts():
    proc = await asyncio.create_subprocess_exec(
        "cargo",
        "run",
        cwd=os.path.join(QUARRY, "quarry-scripts"),
    )
    await proc.wait()


async def run_oracle():
    """
    Run the rust oracle which sets chain tips on the smart contract
    the cwd is the oracle directory and logs are stored in 'oracle/oracle.log'
    """
    is_running = await check_process_running("oracle-service")
    if is_running:
        print("oracle-service already running")
        return

    oracle_path = os.path.join(QUARRY, "oracle")

    proc = await start_pm2_service(
        "oracle-service",
        "cargo",
        "run",
        cwd=oracle_path,
    )
    await proc.wait()

    started_log = "Running "

    print(f"starting oracle-service...")
    await wait_process_running("oracle-service", started_log)
    print(f"oracle-service running")


async def run_stratum():
    """
    Run the SV2 Pool
    cd roles/pool/config-examples
    cargo run -- -c pool-config-local-tp-example.toml

    Run the Job Declarator Server (JDS)
    cd roles/jd-server/config-examples
    cargo run -- -c jds-config-local-example.toml

    Run Job Declarator Client (JDC)
    cd roles/jd-client/config-examples/
    cargo run -- -c jdc-config-local-example.toml

    Run Translator Proxy
    cd roles/translator/config-examples/
    cargo run -- -c tproxy-config-local-jdc-example.toml
    """
    pool_running = await check_process_running("sv2pool-service")
    job_server_running = await check_process_running("sv2jobserver-service")
    job_client_running = await check_process_running("sv2jobclient-service")
    proxy_running = await check_process_running("sv2proxy-service")
    if pool_running and job_server_running and job_client_running and proxy_running:
        print("stratum services already running")
        return
    SV2_BASE = QUARRY + "/../stratum/"
    pool_proc = await start_pm2_service(
        "sv2pool-service",
        "cargo",
        "run",
        "--",
        "-c",
        "pool-config-local-tp-example.toml",
        cwd=SV2_BASE + "roles/pool/config-examples",
    )
    await pool_proc.wait()

    job_server_proc = await start_pm2_service(
        "sv2jobserver-service",
        "cargo",
        "run",
        "--",
        "-c",
        "jds-config-local-example.toml",
        cwd=SV2_BASE + "roles/jd-server/config-examples",
    )
    await job_server_proc.wait()

    job_client_proc = await start_pm2_service(
        "sv2jobclient-service",
        "cargo",
        "run",
        "--",
        "-c",
        "jdc-config-local-example.toml",
        cwd=SV2_BASE + "roles/jd-client/config-examples/",
    )
    await job_client_proc.wait()

    proxy_proc = await start_pm2_service(
        "sv2proxy-service",
        "cargo",
        "run",
        "--",
        "-c",
        "tproxy-config-local-jdc-example.toml",
        cwd=SV2_BASE + "roles/translator/config-examples/",
    )
    await proxy_proc.wait()

    started_log = "Running "

    print("starting sv2pool-service...")
    await wait_process_running("sv2pool-service", started_log)
    print("sv2pool-service running")

    print("starting sv2jobserver-service...")
    await wait_process_running("sv2jobserver-service", started_log)
    print("sv2jobserver-service running")

    print("starting sv2jobclient-service...")
    await wait_process_running("sv2jobclient-service", started_log)
    print("sv2jobclient-service running")

    print("starting sv2proxy-service...")
    await wait_process_running("sv2proxy-service", started_log)
    print("sv2proxy-service running")


def copy_quarry_bindings():
    # Get the directory where the script is running
    script_dir = os.path.dirname(os.path.abspath(__file__))

    # Define the source directory to copy (quarry-sdk)
    source_dir = os.path.join(script_dir, "quarry-sdk")

    # Define the target directory (parent directory's "stratum" folder)
    parent_dir = os.path.dirname(script_dir)
    target_dir = os.path.join(parent_dir, "stratum")

    # Ensure the target directory exists, if not, create it
    if not os.path.exists(target_dir):
        print("stratum directory not found")
        return

    # Define the final destination path
    final_target = os.path.join(target_dir, "quarry-sdk")
    # If the destination directory already exists, remove it
    if os.path.exists(final_target):
        shutil.rmtree(final_target)
        print(f"Previous bindings removed: {final_target}")

    # Copy the directory
    shutil.copytree(source_dir, final_target)
    print(f"Directory copied from {source_dir} to {final_target}")


async def run(
    restart_service,
    list_services,
    kill_services,
    log_service,
    run_all,
    contract,
    oracle,
    stratum,
    copy_bindings,
):
    if copy_bindings:
        copy_quarry_bindings()
        return
    if log_service:
        print(f"Logging {log_service}")
        await display_pm2_logs(log_service)
        return
    if kill_services:
        print("kill running services...")
        subprocess.run(["pm2", "kill"])
        return
    if list_services:
        print("Listing running services...")
        subprocess.run(["pm2", "list"])
        return
    if restart_service:
        print(f"Restarting {restart_service} service...")
        await pm2_delete_process(restart_service)
        if restart_service == "local-evm":
            await run_local_evm()
        elif restart_service == "oracle-service":
            await run_oracle()
        else:
            print(f"Unknown service: {restart_service}")
        return
    if contract or run_all:
        await run_local_evm()
        await deploy_contracts()
    if oracle or run_all:
        await run_oracle()
    if stratum or run_all:
        await run_stratum()


@click.command()
@click.option(
    "--restart-service",
    "-r",
    default=None,
    help="restarts specified service e.g: engine,agent",
)
@click.option(
    "--list",
    "-l",
    "list_services",
    is_flag=True,
    help="Lists running services.",
)
@click.option(
    "--kill",
    "-k",
    "kill_services",
    is_flag=True,
    help="Kill running services.",
)
@click.option(
    "--log",
    "log_service",
    default=None,
    help="Name of the PM2 service whose logs you want to display.",
)
@click.option(
    "--stratum", "-s", "stratum", is_flag=True, help="Run Stratum Mining Pool"
)
@click.option(
    "--contract", "-c", "contract", is_flag=True, help="Run and Deploy Contract"
)
@click.option("--oracle", "-o", "oracle", is_flag=True, help="Run Oracle")
@click.option("--all", "-a", "run_all", is_flag=True, help="Run every service")
@click.option(
    "--copy-bindings",
    "-cb",
    "copy_bindings",
    is_flag=True,
    help="Copy quarry-sdk to Stratum Dir",
)
def main(
    restart_service,
    list_services,
    kill_services,
    log_service,
    run_all,
    contract,
    stratum,
    oracle,
    copy_bindings,
):
    asyncio.run(
        run(
            restart_service,
            list_services,
            kill_services,
            log_service,
            run_all,
            contract,
            oracle,
            stratum,
            copy_bindings,
        )
    )


if __name__ == "__main__":
    main()
