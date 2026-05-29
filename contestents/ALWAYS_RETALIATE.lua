-- name: ALWAYS RETALIATE
-- always defects after first round

function decide(round, history)
    -- On first round, cooperate
    if round == 1 then
        return "cooperate"
    end
    
    -- Always defect after first round
    return "defect"
end
