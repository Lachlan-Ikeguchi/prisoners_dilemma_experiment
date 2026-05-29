-- name: BALL
-- bounces between strategies

function decide(round, history)
    if round % 20 < 10 then return "cooperate" end
    return "defect"
end
