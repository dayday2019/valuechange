#!/bin/bash

for ((i=0; i<100; i++))
do
  # 在这里写你要重复执行的语句
  cargo contract call --contract 5CXGpEYNhGrwhMgqectpLhQrZkDkHK3RH5Db1psaixAzRS8U --message add_score --suri //Alice  --skip-confirm --gas 255292125422
done



