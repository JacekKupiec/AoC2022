# generate first some heights and wind indices to be able to find
# values which can be used as beginning of the cycle.
# I look only for those that are only in cycle. In my example it 6.
# In stock example is 0.

df <- read.csv('..\\output.csv')
result <- lapply(split(1:nrow(df), df$Wind), 
                 function (x) { unique(diff(x)) })
candidates <- as.integer(
  names(
    result[lengths(result) == 1]))