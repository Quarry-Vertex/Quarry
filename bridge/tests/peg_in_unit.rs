// Tests for the peg_in functionality

#[cfg(test)]
mod tests {
    use bridge::peg_in;
    use serde_json::Value;
    use ethers::prelude::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_get_peg_transactions() {
        let client = reqwest::Client::new();
        // test net wallet (on cade's cake wallet)
        let peg_in_addr = "tb1qxmn98awgh46nwjcgq8tt3hwxccysw9we4e78ct";
        let tx_count = 3; // right now just two transactions

        let tx_json: Vec<Value> = peg_in::get_peg_transactions(&client, peg_in_addr)
            .await
            .unwrap();

        // uncomment to show whole json response
        // println!("{:?}", tx_json);
        // correct number of transactions
        assert!(tx_json.len() == tx_count);

        // correct transaction id's
        let mut tx_idi = 0;
        // if this fails there could be more transactions
        let tx_ids = vec![
            "0f2a733f000d577e2df099a7e010e752663916c6287046285d92a51a12ab25ba", // most recent
            "d7d0004a6fa87a451399990c3bbdfea09184a76f28fe8175b864dfcf113e177e",
            "fd47f80b4d91c19d2b744d30dce374c4a3f44233429fbd1005897697dd2b3840",
        ];
        for tx in tx_json {
            assert!(tx["txid"].as_str().unwrap() == tx_ids[tx_idi]);
            tx_idi += 1;
        }
    }

    #[tokio::test]
    async fn test_parse_peg_transactions() {
        let client = reqwest::Client::new();
        // test net wallet (on cade's cake wallet)
        let peg_in_addr = "tb1qxmn98awgh46nwjcgq8tt3hwxccysw9we4e78ct";

        let mut tx_json: Vec<Value> = peg_in::get_peg_transactions(&client, peg_in_addr)
            .await
            .unwrap();

        // should return an empty vec
        let mut latest_tx = "d7d0004a6fa87a451399990c3bbdfea09184a76f28fe8175b864dfcf113e177e";
        // insert OP_RETURN with valid eth address
        for tx in &mut tx_json {
            if let Some(vout_array) = tx["vout"].as_array_mut() {
                for vout in vout_array {
                    if let Some(vout_obj) = vout.as_object_mut() {
                        vout_obj.insert(
                        "scriptpubkey_asm".to_string(),
                        Value::String("OP_RETURN OP_PUSHBYTES_20 a0Ee7A142d267C1f36714E4a8F75612F20a79720".to_string()),
                    );
                    }
                }
            }
        }
        let res_vec = peg_in::parse_transactions(latest_tx, &tx_json, peg_in_addr)
            .await
            .unwrap();
        assert!(res_vec.len() == 0);

        // should return len 1
        latest_tx = "fd47f80b4d91c19d2b744d30dce374c4a3f44233429fbd1005897697dd2b3840";
        let res_vec = peg_in::parse_transactions(latest_tx, &tx_json, peg_in_addr)
            .await
            .unwrap();
        assert!(res_vec.len() == 1);

        // should return len 2
        latest_tx = "";
        let res_vec = peg_in::parse_transactions(latest_tx, &tx_json, peg_in_addr)
            .await
            .unwrap();
        assert!(res_vec.len() == 2);

        // assert that valid eth addressess are being saved
        for pegtx in res_vec {
            let eth_add = Address::from_str("a0Ee7A142d267C1f36714E4a8F75612F20a79720").unwrap();
            assert_eq!(pegtx.eth_address, eth_add);
        }
    }
}
