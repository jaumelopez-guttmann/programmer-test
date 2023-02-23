# QA/Programming hiring test

As a QA/programmer in our team, you will be responsible for maintaining the code quality and functionality, creating and using automated tools to continuously monitor the correct performance of our software. For this test, you will be performing a task that will be your bread and butter at Guttmann: inspect a piece of software and provide feedback to the developers to improve the code quality, and you will build automated tests to verify that the code functions (and that will continue functioning after every change).

This test is based on a number puzzle game, which has been featured on several TV game shows, such as the Spanish "cifras y letras" (La2, 1991-1996). The goal of the game is to obtain a target natural number using a sequence of arithmetic operations, starting with a set of given numbers.  We follow the "Normal Rules" of the digitle website (https://c.eev.ee/digitle/, click the question mark for more details on the rules).

Note that not all possible games have a solution. For instance, you cannot obtain 482 starting with [50, 25, 6, 5, 10]. In that case, the program should output the best possible approximation.

To make the test more interesting, we have introduced one fundamental logic bug in the code you will be reviewing. That means that the bug won't cause any run time error (that's why the developers didn't find this bug before releasing the code to QA), but the program will report that some games don't have a solution, while they actually have a solution that is valid according to the Normal Rules.

_HINT:_ digitle always gives six numbers, but it is actually easier to reproduce the bug on shorter games. However, it's impossible to find this bug in games with 3 or fewer numbers, so we recommend trying 4-numbers games.

_NOTE:_ The code is written in the Rust programming language. It is deliberately written in a language that most people don't know, since we consider that the programming concepts are transversal to the language of choice. Therefore, the test is designed, so it can be completely without being familiar with the details of the Rust syntax. If some parts are a bit difficult to understand, probably did mean that the developer did a poor job deciding variable and function names! As a hint, when you see a “struct”, think of a class in a normal OOP language.
That said, Rust is the main server language in our project, even though most automated tools for testing use other languages. So we will expect you, if you are the successful candidate, to eventually learn Rust. Don't worry, no one in our team spoke Rust when we started.

_HINT:_ Using an IDE that understands Rust is always helpful. If you join our team, we'll provide you and all products pack of Jetbrains IDEs, so you'd use CLion with the Rust plugin. A free alternative is VSCode with the rust plugin.

# What you should do

- Provide feedback on code quality. Are there functions that are too long or difficult to understand? Are there structs (classes) with too many responsibilities? Is there good code cohesion (ie, the functionalities of every code unit are sufficiently related)?
- Create some automated tests to prove that every part of the software is correct. Note that in your final job you'll be focusing on higher levels of the test pyramid, developers will be responsible for unit tests. But since this is a minimal project, you might want to do some unit and integration tests. This resource might be handy: https://doc.rust-lang.org/book/ch11-00-testing.html
- Document any bug that possibly exists (and, as we said, there's at least on big bug that prevents the program to find the correct solution on some games). Make an educated proposal on how to adapt the code design to solve this fundamental bug (and possibly others you find).

# Rubric
We'll assess your performance in 0-10 scale (with some opportunities to gain extra grade).

- **4 points.** Describe the bug that causes some solutions to be unnoticed. What kind of solutions won't be found? What actually causes the bug?
- **1 point.** Provide a recommendation that a developer can actually follow to fix the bug.
- **1 point.** Provide insight on any code quality issue that you find (you'll have some guidelines and style guides in Guttmann. For now, stick with common best practices and principles).
- **4 points.** Provide automated tests to show that the code works (or tests that fail because the code has bugs!).

Extra grade opportunities:

- **2 points.** Be brave enough to fix the bug yourself!
- **1 point.** Create an interactive CLI so the user can enter any game (goal and given numbers) without needing to recompile the program.
- **3 points.** Convert the program into a web server so the user can provide the game details using a Rest API. (Note: We use Actix web for that in our project, but any HTTP crate will work).
- **2 points** (plus the previous). Create a web GUI that consumes the Rest API.

## Lifeline
At any point after being offered the opportunity to take part in the test, and before the 7 days deadline, you may request the "bug example lifeline" to help you in the task. If you request the lifeline, we will provide you an example game that exhibits the bug. Requesting the lifeline will subtract 1 point from your score.
No other feedback will be provided (except clarifications, if needed).

You can also take up to two extra days to answer the test. Every extra day costs 1 point.

# How to submit
You have 7 natural days after receiving the email.
You will normally offer code feedback in a pull request GUI. We do not have one for this test, so you may provide the feedback as comments in the code, or just prepare a Word or Markdown document making clear to what file/line are you referring. All the content must be in English.

We kindly ask you not to publish your work in a public repository (until after the selection process has been finished). A zip file or a tar ball by email would suffice. If you insist on using a (private) git repo, we have accounts you can invite on the major platforms (gitlab, github and bitbucket).

Note that, if you decide to fix the code for the extra credit, it might be beneficial if you submit two versions: one unmodified but with your feedback, and another one with your fixes. 

In any case, you can contact us at the following email: jaumelopez@guttmann.com
