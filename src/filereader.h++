#pragma once
#include <stdio.h>
using namespace std;
class filemanager
{
public:
    filemanager(char* file);
    char* read();
private:
    char* file;
    char* content;
};
