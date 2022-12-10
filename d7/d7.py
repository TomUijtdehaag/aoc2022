def main():
    with open("d7/input.txt") as f:
        cmdlines = f.readlines()
        cmdlines = [line.strip() for line in cmdlines]

    root = Dir("/", None)
    cwd = root
    
    for i, line in enumerate(cmdlines, 1):
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
                            new_dir = Dir(parts[2], cwd)
                            cwd.dirs.append(new_dir)
                            cwd = new_dir

                case "ls":
                    continue

        else:
            if parts[0] == "dir":
                # cwd.dirs.append(Dir(parts[1], cwd))
                pass

            else:
                cwd.files.append(File(parts[1], int(parts[0])))

    print(root.get_size())

    dirs = root.get_subdirs_below(100000)

    for d in dirs:
        print(d)
    print(sum([dir.get_size() for dir in dirs]))
    

class File:
    def __init__(self, name, size) -> None:
        self.name = name
        self.size = size


class Dir:
    def __init__(self, name: str, parent) -> None:
        self.name = name
        self.parent = parent
        self.files = []
        self.dirs = []

    def get_size(self):
        size = 0

        for file in self.files:
            size += file.size

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
            
            else:
                dirs.extend(dir.get_subdirs_below(size))

        return dirs


    def __repr__(self):
        return f"[{self.get_path()}] ({self.get_size()})"


if __name__ == "__main__":
    main()