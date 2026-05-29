-- name: DESPERADO
-- desperate strategy that defects when behind

function decide(round, history)
    if round <= 10 then return "defect" end
    return "cooperate"
end
