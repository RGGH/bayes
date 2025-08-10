# Bayes Theorem in Rust

Imagine there's a rare disease that affects 1 in 1,000 people. There's a test for it that's 99% accurate (it correctly identifies the disease 99% of the time, and correctly says "no disease" 99% of the time when someone is healthy).
Now, you take the test and it comes back positive. What's the chance you actually have the disease?
Your gut might say "99% since the test is 99% accurate!" But Bayes' theorem shows the answer is actually only about 9%.
Why? Because the disease is so rare. Even though the test is very good, there are way more healthy people than sick people. So even with a 1% false positive rate, most positive test results will actually be from healthy people.
The formula:
P(A|B) = P(B|A) × P(A) / P(B)
Where:

P(A|B) = probability of A given that B happened
P(B|A) = probability of B given that A is true
P(A) = initial probability of A
P(B) = total probability of B happening

Why it matters:
Bayes' theorem shows up everywhere - medical diagnosis, spam filtering, artificial intelligence, criminal justice, and scientific research. It's fundamentally about being rational with uncertainty and not jumping to conclusions based on limited evidence.
The key insight is that rare events stay rare even when you have good evidence for them, and you should always consider your "prior" knowledge along with new evidence.
