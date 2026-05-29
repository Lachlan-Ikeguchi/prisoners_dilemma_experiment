-- name: ABSTINENT
-- defects on even rounds, cooperates on odd

function decide(round, history)
    if round % 2 == 0 then
        return "defect"
    else
        return "cooperate"
    end
end
