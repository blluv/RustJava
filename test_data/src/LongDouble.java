class LongDouble {

    public static void main(String[] args) {
        long test1 = 1;
        double test2 = 2;
        int test3 = 3;
        long test4 = 4;

        test1 = testtest(test4);
        test4 = testtesttest(test1);
        test3 = 100;
        testtest(test4);

        System.out.println(test1);
        System.out.println(test2);
    }

    static long testtest(long arg) {
        return arg + 1;
    }

    static long testtesttest(long arg) {
        return arg --;
    }
}
