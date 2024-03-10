near call pichtranbosbounty.testnet create_project "{\"project_name\": \"Project 1\", \"project_description\": \"Description of Project 1\", \"target_amount\": \"100000000000000000000000\", \"ipfs_image\": \"image_link\", \"ipfs_hash\": [\"hash1\", \"hash2\"]}" --accountId pichtrannn.testnet
near view pichtrantestnet.testnet get_projects "{}"

near view pichtrantestnet.testnet get_donors_by_project_id "{"project_id": "pichtrannn.testnet_1710075118849477261_159019043"}"
near view pichtrantestnet.testnet get_donations_by_donor_id '{"donor_id": "pichtrantestnet.testnet"}' --accountId pichtrantestnet.testnet

near view pichtrantestnet.testnet get_donations_on_going --accountId pichtrantestnet.testnet
near view pichtrantestnet.testnet get_donations_ended --accountId pichtrantestnet.testnet
