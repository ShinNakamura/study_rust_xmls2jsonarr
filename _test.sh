#! /bin/bash
unalias -a

test_name="test1"
expect="200"
act="$(cargo run -- tests/input* | jq -r '.[1].result.categories.categoryInfo[0].categoryId')"
if test "_$expect" = "_$act"
then
    echo "$test_name -> pass"
else
    echo  "$test_name -> fail : expect '$expect' but got '$act'"
fi
