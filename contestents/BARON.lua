-- name: BARON
-- defects on even rounds

function decide(round, history)
    if round % 2 == 0 then return "defect" end
    return "cooperate"
end
