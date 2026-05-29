-- name: BAY
-- cooperative with 70% probability

math.randomseed(os.time())

function decide(round, history)
    return math.random() < 0.7 and "cooperate" or "defect"
end
