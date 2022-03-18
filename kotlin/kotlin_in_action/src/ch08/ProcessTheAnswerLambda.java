package ch08;

/* Java */

import static ch08.ProcessTheAnswer.ProcessTheAnswer.processTheAnswer;

public class ProcessTheAnswerLambda {
    public static void main(String[] args) { processTheAnswer(number -> number + 1); }
}