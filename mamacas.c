#include <stdio.h>

int main() {
        
    int choice, quantity;
    long long total, price;
    
    printf("Menu\n");
    printf("1. Poundo yam and edinkaiko soup - N3200\n");
    printf("2. Fried rice and chicken - N3000\n");
    printf("3. Amala and egusi - N2500\n");
    printf("4. Eba and egusi soup - N2000\n");
    printf("5. White rice and stew - N2500\n");
    printf("Enter your choice (as an integer): ");
    scanf("%d", &choice); 
    printf("Enter your quantity: ");
    scanf("%d", &quantity); 


    switch (choice) {
        case 1:
            printf("Poundo yam and edinkaiko soup\n");
            price = 3200;
            total = price * quantity;
            printf("Total is: ");
            printf("%d\n", total);
            break;
        case 2:
            printf("Fried rice and chicken\n");
            price = 3000;
            total = price * quantity;
            printf("Total is: ");
            printf("%d\n", total);
            break;
        case 3:
            printf("Amala and egusi\n");
            price = 2500;
            total = price * quantity;
            printf("Total is: ");
            printf("%d\n", total);
            break;
        case 4:
            printf("Eba and egusi soup\n");
            price = 2000;
            total = price * quantity;
            printf("Total is: ");
            printf("%d\n", total);
            break;
        case 5:
            printf("White rice and stew\n");
            price = 2500;
            total = price * quantity;
            printf("Total is: ");
            printf("%d\n", total);
            break;
        default:
            printf("Invalid choice. Please try again.\n");
            break;
    }
    return 0;
}
    
