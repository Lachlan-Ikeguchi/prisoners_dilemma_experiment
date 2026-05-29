-- name: CLEAN SLATE
-- cooperates with small probability after defection, Tit-for-Tat otherwise

math.randomseed(os.time())

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if history[#history] == "defect" then
        -- 33% chance to forgive and cooperate
        if math.random() < 0.33 then
            return "cooperate"
        else
            return "defect"
        end
    else
        return "cooperate"
    end
end
