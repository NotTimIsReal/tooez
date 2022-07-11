#include "codereader.h++"
#include <vector>
#include <string>
#include <algorithm>
using namespace std;
std::vector<std::string> splitter(string str, string delimiter)
{
    vector<string> result;
    size_t pos = 0;
    string token;
    while ((pos = str.find(delimiter)) != string::npos)
    {
        token = str.substr(0, pos);
        result.push_back(token);
        str.erase(0, pos + delimiter.length());
    }
    result.push_back(str);
    return result;
};
codereader::codereader(std::string pt_content)
{
    content = pt_content;
};
int codereader::run()
{
    std::string c;
    c = content;
    std::vector<std::string> lines = splitter(c, "\n");
    int lineN = 0;
    for (auto line : lines)
    {
        lineN++;
        for (auto t : tokens)
        {
            // check if line contains token
            if (line.find(t) != string::npos)
            {
                if (t == tokens[0])
                {
                    continue;
                }
                if (t == tokens[1])
                {
                    struct variable v;
                    v.name = line.substr(line.find(" ") + 1);
                    v.name = v.name.substr(0, v.name.find(" "));
                    v.value = line.substr(line.find(" ") + v.name.length() + 1);
                    // get rid of all spaces until first character
                    v.value = v.value.substr(v.value.find_first_not_of(" "));
                    v.type = getType(v.value);
                    if (v.type == "unknown")
                    {
                        std::cout << "Unknown type for variable " << v.name << " "
                                  << "at line: " << lineN << std::endl;
                    }
                    variables.push_back(v);
                    break;
                }
                if (t == tokens[2])
                {
                    struct conditionChecker c;
                    std::string condition = line.substr(3);
                    c.item = condition.substr(0, condition.find(" "));
                    c.value = condition.substr(condition.find(" ", c.item.length() + 1) + 1);
                    c.condition = condition.substr(condition.find(" ") + 1, c.value.length() + 1);

                    std::string itemType = getType(c.item);
                    std::string valueType = getType(c.value);
                    struct variable *item = NULL;
                    struct variable *value = NULL;
                    if (itemType == "unknown")
                    {
                        vector<variable>::iterator it = std::find_if(variables.begin(), variables.end(),
                                                                     [&](const variable &o)
                                                                     { return o.name == c.item; });
                        if (it == variables.end())
                        {
                            std::cout << "Unknown variable " << c.item << " "
                                      << "at line: " << lineN << std::endl;
                        }
                        else
                        {
                            int index = std::distance(variables.begin(), it);
                            item = &variables[index];
                            itemType = item->type;
                            c.item = item->value;
                        }
                    }
                    if (valueType == "unknown")
                    {
                        vector<variable>::iterator it = std::find_if(variables.begin(), variables.end(),
                                                                     [&](const variable &o)
                                                                     { return o.name == c.value; });

                        if (it == variables.end())
                        {
                            std::cout << "Unknown variable " << c.value << " "
                                      << "at line: " << lineN << std::endl;
                        }
                        int index = std::distance(variables.begin(), it);

                        value = &variables[index];
                        valueType = value->type;
                        c.value = value->value;
                    }
                    if (itemType != valueType)
                    {
                        std::cout << "Type mismatch for condition " << c.item << " "
                                  << c.condition << " " << c.value << " "
                                  << "at line: " << lineN << std::endl;
                    }

                    break;
                }
            }
        }
    }
};
bool is_number(const std::string &s)
{
    return !s.empty() && std::find_if(s.begin(),
                                      s.end(), [](unsigned char c)
                                      { return !std::isdigit(c); }) == s.end();
}

std::string codereader::getType(std::string str)
{
    if (str.find("true") != string::npos)
    {
        return "bool";
    }
    if (str.find("false") != string::npos)
    {
        return "bool";
    }
    if (str.find("\"") != string::npos)
    {
        return "string";
    }
    if (is_number(str))
    {
        return "number";
    }
    else
    {
        return "unknown";
    }
};
int codereader::evalIf(struct conditionChecker c, std::string codeToBeEvaled, std::string itemType, std::string valueType, int lineN)
{
    if (c.condition == "is")
    {
        if (c.item == c.value)
        {
            // execute do later
        }
    }
    if (c.condition == "isnot")
    {
        if (c.item != c.value)
        {
            // execute do later
        }
    }
    if (c.condition == "morethan")
    {
        if (itemType != "int" || valueType != "int")
        {
            std::cout << "Type mismatch for condition " << c.item << " "
                      << c.condition << " " << c.value << " "
                      << "at line: " << lineN << std::endl;
        }
        else
        {
            if (stoi(c.item) > stoi(c.value))
            {
                // execute do later
            }
        }
    }
    if (c.condition == "lessthan")
    {
        if (itemType != "int" || valueType != "int")
        {
            std::cout << "Type mismatch for condition " << c.item << " "
                      << c.condition << " " << c.value << " "
                      << "at line: " << lineN << std::endl;
        }
        else
        {
            if (stoi(c.item) < stoi(c.value))
            {
                // execute do later
            }
        }
    }
}