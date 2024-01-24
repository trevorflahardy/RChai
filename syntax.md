# Chai Syntax
Some very basic syntax for the Chai language. Will expand as the language grows.

$$
\begin{align}
    % The actual program which contains the list of statements.
    \text{[program]} &\to \text{[statement]}^* \\

    % A statement is either an exit statement or a let statement.
    \text{[statement]} &\to \begin{cases}
        \text{exit([expression]);} \\ % A built in exit function by the compiler.
        \text{let ident = [expression];} \\ 
    \end{cases}\\

    % An expression can be a literal or an identifier.
    \text{[expression]} &\to \begin{cases}
        \text{int\_literal} \\
        \text{ident} \\
    \end{cases}\\
\end{align}
$$

