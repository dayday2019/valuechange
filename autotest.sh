#!/bin/bash

for ((i=0; i<50; i++))
do
  # 在这里写你要重复执行的语句
  cargo contract call --contract 5GB9c32yksL9X97MqKT2RNmj9an4oQeCJzdLdMfH2sxMJaH9 --message add_score --suri //Alice  --skip-confirm --gas 255292125422
done



