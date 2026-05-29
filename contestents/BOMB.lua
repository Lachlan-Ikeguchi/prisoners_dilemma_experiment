-- name: BOMB
-- defects on round 50 and after

function decide(round, history)
    if round >= 50 then
        return "defect"
    else
        return "cooperate"
    end
end
