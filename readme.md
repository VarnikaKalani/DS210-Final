# What is the project?

In this project, we calculate average distances. We pick a large random sample of graph nodes and calculate the distance to all other nodes in the graph. We then output the average distance of all the node pairs.

# Dataset

We used a dataset from kaggle. Following is a link of the dataset: https://www.kaggle.com/datasets/felipetimbo/weighted-networks?select=reality-call.csv

Inside the dataset, there are several graphs. We picked `reality-call.csv` for our analysis. The dataset contains the mobile phone call events between a small set of core users at MIT.

NUMBER OF VERTICES: 6809

NUMBER OF EDGES: 7680

# How to run it?

- Run `cargo run` in your terminal
- You should be good to go after that

# Output

- The program will show as the distances are being calculated
- At the end, the program will display the average distance of all node pairs in the format

```
Average distance: <average distance>
```

# Tests

- The program contains a handful of tests for testing the functionality
- Run them by using `cargo test` in your terminal
