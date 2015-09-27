var searchIndex = {};
searchIndex['lcs'] = {"items":[[0,"","lcs","This crate provides utilities around [least common subsequences][wiki]. From a least common\nsubsequences table, you can also calculate diffs (see `LcsTable::diff`).",null,null],[3,"LcsTable","","",null,null],[4,"DiffComponent","","",null,null],[13,"Insertion","","",0,null],[13,"Unchanged","","",0,null],[13,"Deletion","","",0,null],[11,"fmt","","",1,{"inputs":[{"name":"lcstable"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"diffcomponent"},{"name":"diffcomponent"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"diffcomponent"},{"name":"diffcomponent"}],"output":{"name":"bool"}}],[11,"fmt","","",0,{"inputs":[{"name":"diffcomponent"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","Constructs a LcsTable for matching between two sequences `a` and `b`.",1,null],[11,"longest_common_subsequence","","Gets the longest common subsequence between `a` and `b`. Returned elements are in the form\n`(elem_a, elem_b)`, where `elem_a` is a reference to an element in `a`, `elem_b` is a\nreference to an element in `b`, and `elem_a == elem_b`.",1,{"inputs":[{"name":"lcstable"}],"output":{"name":"vec"}}],[11,"longest_common_subsequences","","Gets all longest common subsequences between `a` and `b`. Returned elements are in the form\n`(elem_a, elem_b)`, where `elem_a` is a reference to an element in `a`, `elem_b` is a\nreference to an element in `b`, and `elem_a == elem_b`.",1,{"inputs":[{"name":"lcstable"}],"output":{"name":"hashset"}}],[11,"diff","","Computes a diff from `a` to `b`.",1,{"inputs":[{"name":"lcstable"}],"output":{"name":"vec"}}]],"paths":[[4,"DiffComponent"],[3,"LcsTable"]]};
initSearch(searchIndex);
