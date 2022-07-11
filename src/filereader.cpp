#include "filereader.h++"
#include <stdio.h>
#include <iostream>
#include <sys/stat.h>
bool exists(const std::string &name)
{
    struct stat buffer;
    return (stat(name.c_str(), &buffer) == 0);
}
filemanager::filemanager(char *pt_file)
{
    file = pt_file;
    FILE *read_file = fopen(file, "r");
    if (!exists(file))
    {
        std::cout << "File not found" << std::endl;
        exit(1);
    }
    if (file == NULL)
    {
        std::cout << ("No Such File Called: ") << file << std::endl;
        exit(1);
    }
    char *buffer = new char[1024];
    fread(buffer, sizeof(buffer), 1024, read_file);
    content = buffer;
}
char *filemanager::read()
{
    return content;
}
