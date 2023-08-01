#include <stdio.h>

main()
{
    // converts C to F

    int min = 0;
    int max = 300;
    int step = 20;
    int calc;

    for (int i = 0; i <= 300; i)
    {
        calc = 9 / 5 * i + 32;
        printf("%d\n", calc);
        i = i + step;
    }
}