[LFX Ffmpeg Plugin](https://github.com/WasmEdge/WasmEdge/issues/2689)

This repository is the solution to First Part of [Pretest #2692](https://github.com/WasmEdge/WasmEdge/discussions/2692)

This is a Lottery Game testing the Luck of Users. 

Trigger the API by sending a Post Request to url: https://code.flows.network/lambda/ilBRz7ybHn. The body should be in JSON format.


##### Example:
```
curl --request POST  -H "Content-Type: application/json" --data '{ "guess" : 2}' https://code.flows.network/lambda/ilBRz7ybHn
```

## Game Rules
* Guess a number between 0-3 inclusive.
* Any number below 0 and greater than 3 throws an error.
* The guess field takes integer data type as input (not string/float/bool).
* You win the lottery when the number guessed is equal to Lottery number.
* Keep calling the API until you win the Lottery.

Results:
### Lucky
![Lucky](https://github.com/Hrushi20/rust-lottery/blob/main/assets/lucky.png)

### Unlucky
![Out Of Luck](https://github.com/Hrushi20/rust-lottery/blob/main/assets/out-of-luck.png)





