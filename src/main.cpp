#include <iostream>
#include "filereader.h++"
#include "codereader.h++"
#include <string>
#include <regex>
#include <algorithm>
#include <iterator>
using namespace std;
int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        std::cout << "Usage: " << argv[0] << " <file>" << std::endl;
        return 1;
    }
    filemanager fr(argv[1]);
    char *content = fr.read();
    codereader cr(content);
    free(content);
    cr.run();
    return 0;
}