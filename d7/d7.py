def main():
    with open("d7/input.txt") as f:
        cmdlines = f.readlines()
        cmdlines = [line.strip() for line in cmdlines]

    root = Dir("ROOT", None)
    cwd = None
    
    for line in cmdlines:
        parts = line.split(" ")

        if line.startswith("$"):
            match parts[1]:
                case "cd":
                    match parts[2]:
                        case "/":
                            cwd = root
                        case "..":
                            cwd = cwd.parent
                        case _:
                            for dir in cwd.dirs:
                                if dir.name == parts[2]:
                                    cwd = dir
                                    break

                            else:
                                raise ValueError("Dir not found")

                case "ls":
                    continue

        else:
            if parts[0] == "dir":
                cwd.dirs.append(Dir(parts[1], cwd))

            else:
                cwd.files.append(File(parts[1], int(parts[0])))

    print(root.get_size())

    dirs = root.get_subdirs_below(100_000)

    for d in dirs:
        print(d)
    print(sum(dirs))

    space_needed = 30_000_000 - (70_000_000 - root.get_size()) 
    print(space_needed)
    dirs = root.get_subdirs_above(space_needed)
    print(sorted(dirs)[0].get_size())


class File:
    def __init__(self, name, size) -> None:
        self.name = name
        self.size = size

    def __repr__(self) -> str:
        return f"File [{self.name}] ({self.size})"

    def __radd__(self, other):
        return self.size + other

class Dir:
    def __init__(self, name: str, parent) -> None:
        self.name = name
        self.parent = parent
        self.files = []
        self.dirs = []

    def __radd__(self, other):
        return other + self.get_size()

    def __lt__(self, other):
        return self.get_size() < other.get_size()

    def get_size(self):
        size = sum(self.files)

        for dir in self.dirs:
            size += dir.get_size()

        return size
    
    def get_path(self):
        if self.parent is not None:
            parent_path = self.parent.get_path()
        else:
            parent_path = ""

        return parent_path + "/" + self.name

    def get_subdirs_below(self, size):
        dirs = []

        for dir in self.dirs:
            if dir.get_size() <= size:
                dirs.append(dir)
            
            dirs.extend(dir.get_subdirs_below(size))

        return dirs

    def get_subdirs_above(self, size):
        dirs = []

        for dir in self.dirs:
            if dir.get_size() >= size:
                dirs.append(dir)
            
            dirs.extend(dir.get_subdirs_above(size))

        return dirs


    def __repr__(self):
        return f"[{self.get_path()}] ({self.get_size()})"


if __name__ == "__main__":
    main()