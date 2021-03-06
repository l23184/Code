#include <iostream>
#include "binaryTree.hpp"
#include <string>

int main(){
    bt_root tree(5);

    tree.add(5);
    tree.add(12);
    tree.add(4);

    tree.add(7);
    tree.add(1);
    tree.add(2);
    tree.add(12);
    tree.add(10);
    tree.add(22);

    tree.printTree();

    return 0;
}
