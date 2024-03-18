near call bosbounty123.testnet create_project "{\"project_name\":\"s\",\"project_description\":\"s\",\"target_amount\":10,\"ipfs_image\":\"s\",\"ipfs_hash\":[\"s\",\"s\"],\"duration\":1}" --accountId pichtrannn.testnet


near call bosbounty123.testnet create_donate "{ \"project_id\": \"pichtrannn.testnet_1710782707207264616\", \"amount\": \"3000000000000000000000000\" }" --accountId pichtrannn.testnet --amount 3 --gas=300000000000000


near view bosbounty123.testnet get_projects


near view bosbounty123.testnet get_donors_by_project_id "{\"project_id\": \"pichtrannn.testnet_1710782707
207264616\"}"

near view bosbounty123.testnet get_donations_by_donor_id "{\"donor_id\":\"pichtrannn.testnet\"}"

