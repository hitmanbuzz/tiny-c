1. *A valid program should always start with a keyword or data-type*.
2. *An expression should always give a value out of it*.

<br>

```math
\begin{aligned}
\text{program} &\rightarrow \textit{function} \mid \textit{variable} \\[1em]

\text{function} &\rightarrow \textit{data\_type} + \textit{name} + \textit{block} \\
\text{var\_decl} &\rightarrow \textit{data\_type} + \textit{name} + \textit{expr} \\
\text{return\_stmt} &\rightarrow \text{return} + \textit{expr} \\
\text{expr\_stmt} &\rightarrow \textit{expr} \\[1em]

\text{data\_type} &\rightarrow \textit{int} \mid \textit{void} \\
\text{name} &\rightarrow \text{a..z} \mid \text{A..Z} + \text{\_} + \text{0..9} + \cdots \\
\cdots &\rightarrow \text{repeat} \\[1em]

\text{block} &\rightarrow [\textit{stmt}] \\
\text{stmt} &\rightarrow \textit{var\_decl} \mid \textit{expr\_stmt} \mid \textit{return\_stmt} \\
\text{expr} &\rightarrow \textit{int\_literal}
\end{aligned}
```