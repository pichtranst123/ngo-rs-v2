near call donatebosbounty.testnet create_project '{"project_name": "Kid go to school","project_description": "This is description", "target_amount": "10","ipfs_image": "image_link_here","ipfs_hash": ["hash1", "hash2"],"duration": 1}' --accountId pichtrannn.testnet

near view pichtranbosbounty.testnet get_projects "{}"

near view pichtranbosbounty.testnet get_donors_by_project_id "{"project_id": "pichtrannn.testnet_1710075118849477261_159019043"}"
near view pichtranbosbounty.testnet get_donations_by_donor_id '{"donor_id": "pichtrantestnet.testnet"}' --accountId pichtrantestnet.testnet

near view pichtranbosbounty.testnet get_donations_on_going --accountId pichtrantestnet.testnet
near view pichtranbosbounty.testnet get_donations_ended --accountId pichtrantestnet.testnet
