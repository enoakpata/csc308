#include <stdio.h>
#include <ctype.h>
#include <string.h>

int main()
{
    char input[20];
    int type = 0;

    printf("Enter an integer, float or character: \n");
    scanf("%s", input);

    if(strlen(input) == 1 && !isdigit(input[0])){
        type = 2; //character
    } else {
        for (int i = 0; i < strlen(input); i++) {
            if(input[i] == '.') {
                type = 1; //float
                break;
            } else if(isdigit(input[i])) {
                continue;
                type = 0; //integer
            } else {
                printf("Invalid input\n");
                return 0;
            }
        }
    }

    switch(type){
        case 0:
            {
                int n;
                sscanf(input, "%d", &n);
                printf("Integer\n");
                printf("Size: %lu  bytes\n", sizeof(n));
                printf("Next four multiples of 3:\n");
                for (int i = 0; i <= 4; i++) {
                    int multiple = n + (3 * i);
                    printf("%d\n", multiple);
                }
            }
            break;
        case 1:
            {
                float f;
                sscanf(input, "%f", &f);
                printf("Float\n");
                printf("Size: %lu  bytes\n", sizeof(f));
                printf("Next four multiples of 3:\n");
                for (int i = 0; i <= 4; i++) {
                    float multiple = f + (3 * i);
                    printf("%f\n", multiple);
                }
            }
            break;
        case 2:
            {
                char c;
                c = input[0];
                printf("Character\n");
                printf("Size: %lu  bytes\n", sizeof(c));
                printf("Next four multiples of 3:\n");
                for (int i = 0; i <= 4; i++) {
                    int multiple = c + (3 * i);
                    printf("%c\n", multiple);
                }
                printf("ASCII: %d\n", c);
            }
            break;
        default:
        printf("Invalid input\n");
    }
    return 0;   
}