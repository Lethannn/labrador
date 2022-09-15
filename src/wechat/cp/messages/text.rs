
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpTextMessage {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Content")]
    pub content: String,
    #[serde(rename="AgentID")]
    pub agent_id: i64,
}

#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
    use super::CpTextMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[text]]></MsgType>\
        <Content><![CDATA[this is a test]]></Content>\
        <MsgId>1234567890123456</MsgId>\
        <AgentID>1</AgentID>\
        </xml>";
        let msg = CpTextMessage::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!("this is a test", &msg.content);
    }
}