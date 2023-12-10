#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    char *ptr = malloc(100);
    if (ptr == NULL)
    {
        printf("Memory allocation failed!\n");
        return 1;
    }

    strcpy(ptr, "This is a string!");
    strcpy(ptr, "This is a message!");
    printf("string: %s\nchar: %c\n", ptr, ptr[0]);
    free(ptr);

    return 0;
}