%option noyywrap

%%
                        
\(                      |
\)                      |
[() ]\K[0-9]+(\.[0-9])* printf("\n%s\n", yytext);
" "                     printf("\n");
.|\n                    printf("%s", yytext);

%%

int main(void)
{
    yylex();
    return 0;
}
