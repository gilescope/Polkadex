ADDRESSES=(5HEhuUC6iyf7J1sPfFmGy8578TE4e2BGcCa6ExuM6eZw9dD9
5G6cVrk7N6rmhGuyVEzZ1DWaQo4fSLZKsr7AxoMJGiaUwZJb
5HKCTPxwu5ESdgzv5sqCPWbVsJv8dDGZjsw8SvbYH3RvYh8N
5HGNmj4yLxPhGRq4bDLegEMWSg1WqEWLNsnx9K7dVSZoXCzG
5CcMhch85NAkyGdRRKvenFBE6E8WK4uaK6dYdU21WDQnz9uF
GkBrxVow98F3qrjhmkfXEcDHvVFF2Njns99v1XSemGqv9d2
EnvuGLnFmGzUwBe3B749FVtxWRqoVV9zhKjXTt5MEy78V8z
5Chf4ksgxVoiSovkSBWYytxCG4GTTiE5PQ2SeZ8De9c9SZ3p
5Eo8pG9BG6qHBumrcBndmJANKU6Rz1oS7UqHhWWKv4vfSMB1
5CfUgFDGdU4jhHMGqXUnDPQjEVb9nuCZYkuTyNE7A2DxW3k3
5HjWkTPsrDyH2JeKHfsvTjKz9UfdG7Pf7NF12Q4FrYZB2uVK
5Gh8j82iH5qSKrsLdBuSfNAqmdnMtAwKk9m524am2LX9sFpa
1CKSg19z68Grh1SGnotebK8cLBTjuG3eENbMAqrRF7WWmum
1BifX6SBDyz6YhKP785PU7Bhxu288xro1W9zLsKSGo1ZQqf
5Di9hpuMsp1hzneeMP9iMVeTd6y4v15bxzANKoCcoGVHhpwb
5CChBZ4SHqhbMxgnRvjrUUPyvcFFS1gGBvfMutYXyREQ9hum
5CHyg59AjmCLMYAPuXhJREDTbWkxZWtSmHmyhLDMMzLQx3yG
5G7NobTQ1AT28MJqkeixeBjMFw55PCDNc2v4cTAWaCFN2WGs
5HRD8htbftZ8ebGdLRQeciRnSkpoGT4V4CkumQZ34vuoZizk
5CFV7mjmLYnfhhYJ923Uk6By4RzbgTYiPjcigq2eysy7psUT
5EEnSdmhSuTcbMwxEWeAKDeUzM9Nt9FZc5NcgQi3RAbdjiMU
5ERf3coWcfhjazcPHqDzy5Wesf7MkvjqGBmJ9fR5eK7VzZVs
5GKJjjmvkPmSUYh6ZL7Rtv2JsZpvr69F6fq5Tyk6rSvNyiem
5H1kGMP7NQGdb1RxCXMzGSYPRs5aLGDYkJyf5wBXoTzavpYm
5DSaSznmKRjt9Lvg4478V54UU8kdNyZKSQM8brSiczFR1S1H
4fZrsA9yjwNbGMjQdSuV1ep11qyYQnwq4DhfaFDoshN4LCad
5FU9QB3d1ULM1GXVxQprNZSvCANxhcNkprFKzaQiKxBvGjAD
5EeXqT2rBwyMYHxEdSgQigi4bXBzC9kCkDP6ovHtEuTrGXzd
5GgBj5JHuQC8cYrbTb92KLye9xtXoMUCB3yh7eRj38x83KvA
5HNRteZJHpcpw6qRP8pDcwFevNcnNTjUsB69zoW3xt9HD7fP
5DVC3ZjVDRc6GP1Yxn6SGUhfFAn9R3csVhPL1BrZMPRF6UYs
5FhYAjFoFiaEhu8yiNwTBum1p8zwUf2gDhBZM6zGpxyGDUeN
5D4xJxZzaG5XR5F2uuXzh1R7RHyfNHBdCeLLYTWKjzjUchcC
5FutF7REdgT7FPbSdK6fwSUjMaS9gimHT6D9jNA5eJWNQNWc
5Eq7Ky4nHFvhXuFDvi1Hr5g5MYDCz3YSSXf5s9ckxLERre27
5FZHxrnTTWxcNM5e2AtXyA2v4mXHYCBwBgswcyNEPPbqGxi1
13u8H6hT7sDbYkibazf3JJMCa19jCzoV8SPrLZx2WXbeX5H6
5HQTGwrFudMNigMA5mNM3VT5dmHgBRpREULR5H7Xw5rDKfFg
5FBzihKVUhGB9iEnmeu5b6yXtcTnJQUZbd5fK7SGzEZzHDXm
5D4vinTCBWPiJ3YYsJLdiYfiMjsmSTcgVstjzyire56Mb3Jj)

for i in "${ADDRESSES[@]}"
do
   subkey inspect "$i" | grep 'Public key (hex)'|cut -f2 -d ":"
done