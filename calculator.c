#include <stdio.h>

int addition(float a, float b) {
    return a + b;
}

int subtraction(float a, float b) {
    return a - b;
}

int division(float a, float b) {
    return a / b;
}

int multiplication(float a, float b) {
    return a * b;
}

int main() {
    int choice;
    float num1, num2;

    printf("Select operation:\n");
    printf("1. Add\n");
    printf("2. Subtract\n");
    printf("3. Multiply\n");
    printf("4. Divide\n");
    printf("Enter choice (1/2/3/4): ");
    scanf("%d", &choice);

    if (choice == 1) {
        printf("Enter first number: ");
        scanf("%f", &num1);
        printf("Enter second number: ");
        scanf("%f", &num2);
        addition(num1, num2);
    } else if (choice == 2) {
        printf("Enter first number: ");
        scanf("%f", &num1);
        printf("Enter second number: ");
        scanf("%f", &num2);
        subtraction(num1, num2);
    } else if (choice == 3) {
        printf("Enter first number: ");
        scanf("%f", &num1);
        printf("Enter second number: ");
        scanf("%f", &num2);
        multiplication(num1, num2);
    } else if (choice == 4) {
        printf("Enter first number: ");
        scanf("%f", &num1);
        printf("Enter second number: ");
        scanf("%f", &num2);
        division(num1, num2);
    } else {
        printf("Invalid choice! Please select a valid operation.\n");
    }

    return 0;
}