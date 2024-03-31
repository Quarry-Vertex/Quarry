import click
import asyncio
import subprocess
import json
import os

PIPE = asyncio.subprocess.PIPE

QUARRY = os.path.dirname(os.path.abspath(__file__))
SERVICES = ["oracle"]


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


async def run_local_evm() -> None:
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
    is_running = await check_process_running("oracle-service")
    if is_running:
        print("oracle-service already running")
        return
    oracle_path = os.path.join(QUARRY, "oracle")
    await asyncio.create_subprocess_exec(
        "pm2",
        "start",
        "cargo",
        "--name",
        "oracle-service",
        "--",
        "run",
        cwd=oracle_path,
    )

    print(f"starting oracle-service...")
    print(f"oracle-service running")


async def run(restart_service, list_services, kill_services):
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
    await run_local_evm()
    await deploy_contracts()
    await run_oracle()


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
def main(restart_service, list_services, kill_services):
    asyncio.get_event_loop().run_until_complete(
        run(restart_service, list_services, kill_services)
    )


if __name__ == "__main__":
    main()
