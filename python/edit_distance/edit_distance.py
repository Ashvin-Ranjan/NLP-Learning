import sys

DELETE_COST = 1
INSERT_COST = 1
SUBSTITUTION_COST = 2

def edit_distance(source, target):
    # It has to be like this because multiplying arrays by a scalar causes memory reference issues
    distance_matrix = [[0]*(len(target)+1) for _ in range(len(source)+1)]
    for i in range(1, len(target)+1):
        distance_matrix[0][i] = distance_matrix[0][i-1] + INSERT_COST
    for i in range(1, len(source)+1):
        distance_matrix[i][0] = distance_matrix[i-1][0] + DELETE_COST
    for i in range(1, len(source)+1):
        for j in range(1, len(target)+1):
            distance_matrix[i][j] = min(
                distance_matrix[i-1][j] + DELETE_COST,
                distance_matrix[i][j-1] + INSERT_COST,
                 distance_matrix[i-1][j-1] + (0 if source[i-1] == target[j-1] else SUBSTITUTION_COST)
            )
            
    return distance_matrix[-1][-1]

if __name__ == "__main__":
    print(edit_distance(sys.argv[1].lower(), sys.argv[2].lower()))