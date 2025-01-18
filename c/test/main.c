#include <stdio.h>
#include <sys/signal.h>

int get_area(int width, int height) { return width * height; };

float get_user_score_percentage(int maximum_score, int user_current_score) {
  return (float)user_current_score / maximum_score * 100;
};

int main(int argc, char *argv[]) {
  // this is a comment.
  printf("Hello World\n\t"); // this is also a comment.
  printf("Hello!\t");

  /*
   * This is also a comment but
   * this is a multiple-line comment.
   */
  printf("Have a good day!\n");

  int something_in_integer_type = 20;

  printf("something_in_integer_type = %d\n", something_in_integer_type); // 20

  // printf(something_in_integer_type); // nothing will happenI

  float something_in_floating_type = 20.0;
  char something_in_char_type = 'h';

  something_in_integer_type = 30;

  printf("var = %d\n", something_in_integer_type); // 30

  int a;

  // a variable can be defined first and declared later.
  a = 20;

  // specifier
  //  d or i for int,
  //  c for char,
  //  f or F for float,
  //  lf for double,
  //  lu for unsigned integer
  printf("%d\n", a);
  printf("%c\n", something_in_char_type);
  printf("%f%d%c\n", something_in_floating_type, a, something_in_char_type);

  printf("%c\n", '2');

  int x = 5;
  int y = 10;
  int z = x + y;

  printf("%d\n", z);

  // define
  int b = 20;

  // overwrite
  b = 30;

  // cannot be re-defined
  /* int b = 20; */

  int q = 10, w = 30, e = 52, r = 11;

  printf("%d%d%d%d\n", q, w, e, r);

  int u, i, o;
  u = i = o = 20;
  printf("%d\n", u + i + o);

  int width = 20, height = 100;

  printf("The total area is %d\n", get_area(width, height));

  double something_in_double_type = 20.1234;

  printf("%lf\n", something_in_double_type);

  char aa = 65, bb = 66, cc = 67; // !!!!!
  printf("%c", aa);
  printf("%c", bb);
  printf("%c\n", cc);

  char text[] = "Hello world!";

  printf("%s\n", text);

  float num = 35e3; // 35,000

  float ii = 100.00;
  double ia = 20.23428000481904890; // stdio printf function can print up to
                                    // sixth decimal point.

  printf("%f\n", ii);
  printf("%.1f\n", ii);
  printf("%lf\n", ia);
  printf("%.1f\n", ii);
  printf("%.5lf\n", ia);

  ia = 19.99999;
  printf("%.0lf\n", ia);

  int aaa;                    // 4 bytes
  float bbb;                  // 4 bytes
  double ccc;                 // 8 bytes
  char ddd;                   // 1 bytes
  char eee[] = "hello world"; // 11 bytes...??

  printf("%lu\n", sizeof(aaa));
  printf("%lu\n", sizeof(bbb));
  printf("%lu\n", sizeof(ccc));
  printf("%lu\n", sizeof(ddd));
  printf("%lu\n", sizeof(eee));

  float myFloat = 9; // automatically prints 9.000000
  int myInt = 9.99;  // automatically prints 9, getting rid of .99

  printf("%f\n", myFloat);

  int xxx = 5, yyy = 2;
  float sum = xxx / yyy;

  printf("%d\n", xxx);
  printf("%d\n", yyy);
  printf("%d\n", sum); // prints 2 because the sum is in the integer type. Since
                       // xxx and yyy are in the integer type, the final value
                       // is also integer type, not 2.5, which is float.

  float anotherSum = (float)xxx / yyy; // Explicit conversion happens manually

  printf("%.1f\n", anotherSum); // 2.5!!!

  printf("%.3f\n", get_user_score_percentage(100, 40));
  printf("%.3f\n", get_user_score_percentage(200, 1));

  const int MY_AGE = 17;

  printf("%d\n", MY_AGE);

  // it is not possible
  // myAge = 20;

  return 0;
}
