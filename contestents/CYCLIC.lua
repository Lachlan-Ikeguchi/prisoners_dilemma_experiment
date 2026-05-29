-- name: CYCLIC
-- cycles through a pattern: cooperate, cooperate, defect

function decide(round, history)
    local cycle = (round - 1) % 3
    if cycle == 0 or cycle == 1 then
        return "cooperate"
    else
        return "defect"
    end
end
