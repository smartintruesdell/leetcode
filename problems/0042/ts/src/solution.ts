export class Solution {
    public static solve(numbers: number[], numbersSize: number, numbersToMerge: number[], numbersToMergeSize: number): void {
        let writeIdx = numbersSize + numbersToMergeSize - 1;
        let i = numbersSize - 1;
        let j = numbersToMergeSize - 1;

        while (i >= 0 && j >= 0) {
            if (numbers[i] > numbersToMerge[j]) {
                numbers[writeIdx] = numbers[i];
                i -= 1;
            } else {                
                numbers[writeIdx] = numbersToMerge[j];
                j -= 1;
            }
            writeIdx -= 1;            
        }

        while (j >= 0) {
            numbers[writeIdx] = numbersToMerge[j];
            j -= 1;
            writeIdx -= 1;
        }
    }
}