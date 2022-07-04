

struct MD5HashCashExecution{
    input: MD5HashCashInput,
    output: MD5HashCashOutput,
}

impl MD5HashCashExecution {

    fn findSeed(&self) -> u64 {
        8
    }

    fn checkSeed(&self) -> bool {
        true
    }



}