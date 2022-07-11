#pragma once
#include <stdio.h>
#include <iostream>
#include <string>
#include <vector>
struct variable
{
    std::string name;
    std::string value;
    std::string type;
};
struct conditionChecker
{
    std::string item;
    std::string condition;
    std::string value;
};
class codereader
{
public:
    codereader(std::string pt_content);
    int run();

private:
    const std::vector<std::string> tokens = {"ignore", "make", "if", "else", "elif", "while", "return", "func", "end"};
    std::string content;
    std::vector<variable> variables;
    const std::vector<std::string> conditions = {"is", "isnot", "morethan", "lessthan"};
    std::string getType(std::string str);
    int evalIf(struct conditionChecker c, std::string codeToBeEvaled, std::string itemType, std::string valueType, int lineN);
};
