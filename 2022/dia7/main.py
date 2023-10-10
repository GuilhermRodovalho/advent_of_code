from typing import List, Optional


class FSNode:
    def __init__(self, name, parent, size, type) -> None:
        self._name = name
        self.parent = parent
        self._children = []
        self._size = size
        self._type = type

    @property
    def get_size(self):
        if self._type == "file":
            return self._size
        else:
            return sum([child.get_size for child in self._children])


def read_dir(parent_node: FSNode, lines) -> None:
    for line in lines:
        if line.startswith("dir"):
            name = line[3:].strip()
            node = FSNode(name, parent_node, 0, "dir")
            parent_node._children.append(node)
        else:
            size, name = line.split()
            node = FSNode(name, parent_node, int(size), "file")
            parent_node._children.append(node)


def main(lines):
    last_dir = FSNode("/", None, 0, "dir")
    print("root node created")
    for index in range(len(lines)):
        if index == 0:
            continue
        line = lines[index]
        if line == "$ ls":
            line_of_next_instructon = find_line_of_next_instruction(lines[index + 1 :])
            if line_of_next_instructon == -1:
                read_dir(last_dir, lines[index + 1 :])

            read_dir(last_dir, lines[index + 1 : index + line_of_next_instructon + 1])
        elif line == "$ cd ..":
            assert last_dir
            last_dir = last_dir.parent
            assert last_dir
        elif line.startswith("$ cd "):
            next_dir_name = line.split()[-1]
            last_dir = find_dir(last_dir, next_dir_name)
            assert last_dir

    # sum the size of all dirs that are < 100000
    root = find_root(last_dir)
    # children_to_lookup = [root]
    # total = 0
    # print("summing the size of all dirs that are < 100000")
    # while children_to_lookup:
    #     child = children_to_lookup.pop()
    #     if child._type == "dir":
    #         if child.get_size < 100000:
    #             total += child.get_size
    #         children_to_lookup.extend(child._children)

    # print(total)

    """
    Finding the smallest directory that when removed
    x = 70000000 - size_of_root 
    30000000 <= x + smallest_dir 
    """
    # print root data
    print("root data:")
    print(root._name)
    print(root.get_size)
    size_of_root = root.get_size

    free_space = 70000000 - size_of_root
    children_to_lookup = [root]
    possible = []
    while children_to_lookup:
        child = children_to_lookup.pop()
        if child._type == "dir":
            if (child.get_size + free_space) >= 30000000:
                possible.append(child)
            children_to_lookup.extend(child._children)

    print("free space:", end=" ")
    print(free_space)
    print("size of root:", end=" ")
    print(size_of_root)
    print("Finding the smallest directory that when removed")
    print(f"free space = 70000000 - size_of_root = {free_space}")
    print("smallest dir:", end=" ")
    dir = min(possible, key=lambda x: x.get_size)
    print(dir._name)
    print(dir.get_size)

    # print file tree
    # print("file tree:")
    # print_file_tree(root)


def print_file_tree(node: FSNode, indent=0):
    print(" " * indent, end="")
    print(node._name, end=" ")
    print(node.get_size)
    for child in node._children:
        print_file_tree(child, indent + 2)


def find_dir(node: FSNode, name: str) -> Optional[FSNode]:
    for child in node._children:
        if child._name == name:
            return child

    return None


def find_root(node: FSNode) -> FSNode:
    while node.parent:
        node = node.parent

    return node


def find_line_of_next_instruction(lines: List[str]) -> int:
    res = 0
    for line in lines:
        if line.startswith("$"):
            return res
        res += 1

    return -1


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        lines = f.readlines()
        lines = list(map(lambda x: x.strip(), lines))

    main(lines)
