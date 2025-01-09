import { Solution } from './solution';

describe('Solution', () => {
    it('should solve the problem', () => {
        const numbers = [1, 2, 3, 0, 0, 0];
        const numbersSize = 3;
        const numbersToMerge = [2, 5, 6];
        const numbersToMergeSize = 3;

        Solution.solve(numbers, numbersSize, numbersToMerge, numbersToMergeSize);

        expect(numbers).toEqual([1, 2, 2, 3, 5, 6]);
    });
});