-- name: DECEIVER
-- starts nice then betrays

function decide(round, history)
    if round <= 10 then return "cooperate" end
    return "defect"
end
