1. *A valid program should always start with a keyword or data-type*.
2. *An expression should always give a value out of it*.

<br>

$$

\text{program} \rightarrow \textit{function} \space | \space \textit{variable} \\[1em]

\text{function} \rightarrow \textit{data\_type} \space + \textit{name} \space + \textit{block} \\
\text{var\_decl} \rightarrow \textit{data\_type} \space + \textit{name} + \space \textit{expr} \\
\text{return\_stmt} \rightarrow \text{return} + \space \textit{expr} \\
\text{expr\_stmt} \rightarrow \textit{expr} \\[1em]

\text{data\_type} \rightarrow \textit{int} \space | \space \textit{void} \\
\text{name} \rightarrow \text{a..z} \space | \space \text{A..Z} + \space \text{\_} + \space \text{0..9} + \space \text{...\space *} \\
\text{*} \rightarrow \text{repeat} \\[1em]

\text{block} \rightarrow \text{[\space \textit{stmt} \space]} \\
\text{stmt} \rightarrow \textit{var\_decl} \space | \space \textit{expr\_stmt} \space | \space \textit{return\_stmt} \\
\text{expr} \rightarrow \textit{int\_literal}

$$