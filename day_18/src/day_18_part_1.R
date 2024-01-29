lines <- readLines("../input.txt")
cubes <- t(sapply(strsplit(lines, ","), as.integer))
distances <- as.matrix(dist(cubes, method = "manhattan"))

all_surfaces <- nrow(cubes) * 6
answer <- all_surfaces - sum(ifelse(distances == 1, 1, 0))
print(answer)
