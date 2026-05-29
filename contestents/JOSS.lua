-- name: JOSS
-- Tit-for-Tat with occasional forgiveness

math.randomseed(os.time())

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if history[#history] == "defect" then
        -- 33% chance to forgive
        if math.random() < 0.33 then
            return "cooperate"
        else
            return "defect"
        end
    else
        return "cooperate"
    end
end
