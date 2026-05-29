-- name: COHEN
-- Tit-for-Tat variant that occasionally cooperates after defect

math.randomseed(os.time())

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if history[#history] == "defect" then
        -- 10% chance to cooperate after being betrayed
        if math.random() < 0.1 then
            return "cooperate"
        else
            return "defect"
        end
    else
        return "cooperate"
    end
end
