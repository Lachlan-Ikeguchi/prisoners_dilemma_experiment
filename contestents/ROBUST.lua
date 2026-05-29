-- name: ROBUST
-- Tit-for-Tat with noise tolerance

math.randomseed(os.time())

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- 5% chance to forgive a defection
    if history[#history] == "defect" and math.random() < 0.05 then
        return "cooperate"
    end
    
    return history[#history]
end
