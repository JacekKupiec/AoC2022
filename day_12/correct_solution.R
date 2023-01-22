# The code is taken from: https://www.reddit.com/r/adventofcode/comments/zjnruc/2022_day_12_solutions/

# 2022 Day 12

input <- readLines("D:\\source\\Rust\\AoC 2022\\day_12\\input.txt")
input <- lapply(input, strsplit, split = "")
input <- matrix(unlist(input), nrow = length(input), byrow = TRUE)

Start <- which(input == "S")
End <- which(input == "E")

number_map <- matrix(nrow = dim(input)[1], ncol = dim(input)[2], 0)
for (i in 1:26){
  number_map[which(input == letters[i])] <- i
}
number_map[Start] <- 1
number_map[End] <- 26

pathfinder_forwards <- function(Start) {
  # Setup for Dijkstra's Algorithm
  dims <- dim(number_map)
  distance <- matrix(nrow = dims[1], ncol = dims[2], Inf)
  distance[Start] <- 0
  unvisited <- matrix(nrow = dims[1], ncol = dims[2], 1)
  
  # Dijkstra's Algorithm
  current <- Start
  while (unvisited[End] != 0) {
    current <- which(distance == min(distance[which(unvisited == 1)]) & unvisited == 1)[1]
    currentAI <- arrayInd(current, dims)
    adjacent_inds <- data.frame(rbind(currentAI + c(0, 1), currentAI + c(1, 0), currentAI - c(0, 1), currentAI - c(1, 0)))
    adjacent_inds <- subset(adjacent_inds, X1 > 0 & X1 <= dims[1] & X2 > 0 & X2 <= dims[2])
    connected_verts <- (adjacent_inds[, 2] - 1) * (dims[1]) + adjacent_inds[, 1]
    connected_verts <- connected_verts[which(number_map[connected_verts] < number_map[current] + 2)]
    for (i in seq_along(connected_verts)) {
      j <- connected_verts[i]
      distance[j] <- min(distance[j], distance[current] + 1)
    }
    unvisited[current] <- 0
    current <- which(distance == min(distance[which(unvisited == 1)]) & unvisited == 1)[1]
  }
  return(distance[End])
}

pathfinder_backwards <- function(End) {
  lowest_points <- which(number_map == 1)
  
  # Setup for Dijkstra's Algorithm
  dims <- dim(number_map)
  distance <- matrix(nrow = dims[1], ncol = dims[2], Inf)
  distance[End] <- 0
  unvisited <- matrix(nrow = dims[1], ncol = dims[2], 1)
  
  # Dijkstra's Algorithm
  current <- End
  while (length(which(unvisited == 1)) > 0) {
    current <- which(distance == min(distance[which(unvisited == 1)]) & unvisited == 1)[1]
    currentAI <- arrayInd(current, dims)
    adjacent_inds <- data.frame(rbind(currentAI + c(0, 1), currentAI + c(1, 0), currentAI - c(0, 1), currentAI - c(1, 0)))
    adjacent_inds <- subset(adjacent_inds, X1 > 0 & X1 <= dims[1] & X2 > 0 & X2 <= dims[2])
    connected_verts <- (adjacent_inds[, 2] - 1) * (dims[1]) + adjacent_inds[, 1]
    connected_verts <- connected_verts[which(number_map[connected_verts] > number_map[current] - 2)]
    for (i in seq_along(connected_verts)){
      j <- connected_verts[i]
      distance[j] <- min(distance[j], distance[current] + 1)
    }
    unvisited[current] <- 0
  }
  return(min(distance[lowest_points]))
}

print(pathfinder_forwards((Start)))
print(pathfinder_backwards(End))