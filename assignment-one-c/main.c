#include <stdio.h>
#include <stdlib.h>
#include <string.h>
 
int main()
{
    FILE* ptr;
    char ch;
 
    ptr = fopen("../input.txt", "r");
 
    if (NULL == ptr) {
        printf("file can't be opened \n");
    }
 
    printf("content of this file are \n");
 
    int index = 0;
    for (char ch = fgetc(ptr); ch != EOF; ch = fgetc(ptr)) {
        printf("%c", ch);
        printf("%d", index);
        index++;
    }
 
    fclose(ptr);
    return 0;
}